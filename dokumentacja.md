# Generacja obrazów - przykład łączenia Pythona z Rustem

## Wprowadzenie

Przykład podzielony jest na trzy części - główny program w języku Python zawarty w pliku `main.py` oraz używane przez niego moduły - `pyfract` napisany w Pythonie i  `rsfract` napisny w języku Rust.

W celu połączenia programów w Pythonie i Ruscie wykorzystałem crate [`pyo3`](https://github.com/PyO3/pyo3) po stronie Rusta oraz pakiet do Pythonona [`maturn`](https://github.com/PyO3/maturin) pozwalający na budowanie i publikowanie programów napisanych w Ruscie i wykorzystujących `pyo3` jako pakiety Pythonowe.

## Budowanie i uruchamianie przykładu

Aby moć korzystać z pakietu `maturin`, należy najpierw utworzyć środowisko wirtualne. Można to zrobić poprzez komendę

```
python3 -m venv venv
```

Następnie w zależności od systemu operacyjnego możemy aktywować środowisko wirtualne poprzez komendy:

Linux
```
source venv/bin/activate
```

Windows (PowerShell)
```
.\venv\Scripts\Activate.psl
```

Windows (cmd)
```
.\venv\Scripts\activate.bat
```

I zainstalować wymagane pakiety

```
pip3 install -r requirements.txt
```

W celu uruchomienia przykładu należy najpierw zbudować moduł w Ruście. W tym celu należy przejść do podfolderu `rsfract` i skorzystać z komendy

```
maturin develop
```

Następnie po zakończeniu budowania możemy uruchomić przykład poprzez uruchomienie programu `main.py`

```
python3 main.py
```

Na systemie Linux można również zrobić to za pomocą przygotowanego skrypu

```
./build_and_run.sh
```

## Funkcjonalność programu

Program po uruchomieniu wyświetli prosty interfejs tekstowy pozwalający na wykonanie jednej z dwóch operacji - wygenerowanie obrazu szumu losowego lub zbioru Mandelbrota. Po wybraniu opcji program w Pythonie woła odpowiednie funkcje, napisane w Pythonie oraz pochodzące z modułu napisanego w Ruście, a następnie wypisuje do konsli czas wykonania każdej z mnich i wyświetla wygenerowane obrazy. 

## Szczegóły implementacji modułu w Ruście

### Integracja na linii Python - Rust

Pisząc w Ruście funkcje, która ma być dostępna z poziomu Pythona przy pomocy `pyo3` należy zwócić uwagę na kilka rzeczy:

- typy danych - funkcje powinny zwracać odpowiednie typy danych, aby owe dane mogły być używane w Pythonie. Poza podstawowym stworzonym w tym celu typem `PyResult` mogą to być również na przykład szeroko używane w tym przykładzie typy danych z crate'a `numpy`, który zgodnie z nazwą zawiera funkcje i dane analogiczne do pakietu `numpy` w Pythonie.  
- lifetime - zwracając dane przez referencje należy zwórić uwagę na to, by ich *lifetime* był taki sam jak *lifetime* interpretera Pythona. W praktyce sprowadza się to do tego, że pierwszym argumentem każdej funkcji w module z adnotacją `#[pymodule]` jest `py: Python<'py>`, a w samej funkcji używamy *lifetime*'u `'py` wszędzie gdzie to potrzebne.  
- struktura programu - należy stosować się do konwencji `pyo3` i używać opdowiednich *attribute macros* przy implementowanych funkcjach.  

Więcej o `pyo3` można dowiedzieć się z [`oficjalnej dokumentacji`](https://pyo3.rs/v0.17.3/).

### Funkcje generujące szum losowy

Funkcja generująca szum losowy w Ruście z premedytacją została napisana na kilka sposobów:

`generate_noise` - wariant najprostszy, wykorzystuje pętle for i po kolei losuje wartości kolorów dla pikseli  
`generate_noise_threaded` - wariant ten wykonuje obliczenia na kilku wątkach, a następnie konkatenuje tablice z wynikami w tablicę wynikową reprezentującą wygenerowany obraz  
`generate_noise_threaded_with_locks` - ten wariant również wykorzystuje wątki, ale wykonuje obliczenia na jednej tablicy korzystając z mechanizmów dostępu do zasobów współdzielonych, takich jak mutexy i strefa krytyczna  
`generate_noise_parallel` - ten wariant wykorzystuje crate `rayon` do zrównoleglania wobliczeń wykonywanych na tablicy reprezentującej obraz  

### Wnioski wydajnościowe

W testowanych przypadkach najlepsze wyniki osiągnęła funkcja `generate_noise_threaded`, ponieważ rozkłada ona obliczenia na kilka wątków równocześnie nie korzystając ona z mutexów i blokowania. Najgorzej radzą sobie funkcje `generate_noise_threaded_with_locks` oraz `generate_noise_parallel` - znacznie gorzej nawet od wariantu podstawowego. Prawdopodbnie spowodowane jest to narzutem czasowym spowodowanym przez mechanizmy zapewniające synchronizację dostępu do zasobów dzielonych.
