GPW Challenge
==============

To jest implementacja rozwiazania problemu w ramach rekrutacji na stanowisko programisty w Giełdzie Paperów Wartościowych (czerwiec 2020).
Ćwiczenie zostalo zamknięte, nie przewiduję żadnego rozwoju tego kodu.

Opis zadania
--------------
Cyfrowy labirynt o szerokości X i długości Y składa się z zer i jedynek, gdzie zero jest ścianą
a jedynka drogą. Napisz aplikację w języku Rust, która znajdzie drogę do wyjścia z labiryntu o
najmniejszej liczbie zakrętów.

Rozwiązanie
-------------
Problem starałem się zamodelować przy pomocy grafu z wagami przejść (i możliwymi cyklami), tak żeby zastosować jeden ze znanych
algorytmów szukania optymalnej drogi - np. algorytm Dijkstra'y.

W przypadku tego specyficznego problemu:
- węzłami są wszytkie punkty zadeklarowane jako "otwarte", t.j. `1` w pliku wejsciowym
- waga przejscia między węzłami nie jest stała, ale jest wyliczana dynamicznie na podstawie dotychczasowej drogi (czyli poprzedniego węzła)
  i wynosi 0 jezeli nie zmieniamy kierunku lub 1 przy zmianie kierunku.

Dla ułatwienia dalej będę używał pojęć:
- "najprostsza droga" dla oznaczenia drogi z najmniejszą liczbą zakrętów (czyli cel zadania).
- "ranking drogi" - liczba zakrętów
- "ranking wązła" - ranking znanej drogi do tego wązła z punktu początkowego

Na początku procedury program umieszcza wszystkie wezly w kontenerze zwanym `world`. To odpowiada stanowi "nie zmany żadnej drogi do tego punktu".
Wyjątek stanowi punkt startowy, kóry jest umieszczany w kontenerze `heaven` wraz z rankingiem najprostszej drogi (0 w tym wypadku) oraz poprzednim węzłem na tej drodze (samym soba).
Tworzymy tez pusty kontener `purgatory` (czyściec).

Zaczynamy od wązła startowego. W każdym kroku jest jeden bierzący wązeł który juz znajduje się w `heaven` i sprawdzamy wszystkie sąsiednie węzły:

- jeżeli sąsiad jest już w `heaven` to pomijamy
- jeżeli sąsiad jest w `purgatory`, to wyliczamy jego ranking na postawie drogi przez aktualny węzeł (ranking aktualnego węzła ew. powiększony o jeden jeżeli jest zakręt)
  i porównujemy ze znanym rankingiem tego sąsiada (pamietanym w `purgatory`) - jezeli jest mniejszy (lepszy) to go zapisujemy oraz zapisujemy bieżący węzel jako poprzedni dla sąsiada.
- w pozostałym wypadku, wyliczamy ranking sąsiada oraz poprzedni węzeł (tak jak poprzednio) i wpisujemy te informacje do `purgatory`.

Nastąpnie spośród wązłów w `purgatory` wybieramy ten z najmniejszym rankingiem, oraz:

- jeżeli nie ma takiego (`purgatory` jest puste), to wychodzimy
- jezeli minimalny jest końcowym, to przenosimy go do `heaven` i wychodzimy
- w pozostałym wypadku przenosimy minimalny węzeł do `heaven` wraz z rankingiem i adresem poprzednego, ustalamy ten wązał jako bierzący i idziemy do począku (sprawdzanie sąsiadów)

Na końcu sprawdzany czy węzeł końcowy znajduje sie w `heaven`, jezeli tak, to pobieramy jego ranking i podajemy jako roazwiązanie.
Jezeli nie ma go tam, to nie ma rozwiązania - czyli nie ma drogi z począktu do końca.

Wezły przechodzą tylko w jednym kierunku: `world` -> `purgatory` -> `heaven`. Ponadto w każdym kroku usuwamy jeden wązeł z `purgatory`. 
Oznacza to, że procedura nie zapętli się. Ponadto ustawiany ranking sąsiada jest zawsze >= rankingowy bieżacego, oraz jako bieżacy bierzemy "słabe minimum", 
to oznacza, że w każdym kojenym kroku do końca procedury nie ustawimy rankingu niższego niż bierzący. Jednocześnie każdy bieżacy węzeł jest w `heaven`, 
co oznacza że te węzły maja najmniejszy możliwy ranking - w tym ew. wązeł końcowy.

Szacunkowa złożonośc obliczeniowa wynosi ~ 4*(X*Y)^2

Program
--------
Głowny program pobiera dane w zadanym formacie ze standardowego wejścia i drukuje wynik (liczba zakrętów) na standartowe wyjście.
Program zwraca 0 jeżeli jest wynik, -1 jezeli nie ma rozwiązania, -2 jeżeli dane są nieprawidłowe, zbyt duże, lub nastąpił inny bład.
Komunikaty o błędach są drukowane na `stderr`. Wyjście poprzez 'panic' moze nastąpić jedynie w razie jakiegoś niewykrytego błędu logicznego w programie.
Wstąpna weryfikacja danych powinna wykluczyć błedy typu overflow (jezeli coś mi nie umsknąło).
Uruchomienie programu z opcją `--dbg` powoduje wydruk na `stdout` ścieżki rozwiązania (listę węzłów).
Uruchomienie programu z inna (dowolną) opcją, zgodnie z zadaniem powoduje potraktowanie linii wejściowych jaki liczb w notacji binarnej i zamiane na dziesiętna.

Algorytmy
----------
Są zakodowane 2 podobne alrytmy zgodnie z powższym opisem.
Różnią się one użyciem tablicy do zapamiętania pozycji i relacji wezłów (`dijkstra_speed`) v.s. hash set (`dijkstra_mem`).
Zamysłem tego drugiego, bylo przechowywanie tylko istotnych (1-wych) wezłów, co mało korzystnie wpłynąć na zurzycie pamięci.
W praktyce jednak sie to nie potwierdziło - prawdopodobnie ze wzgledu na i tak duzy stosunek 1 do 0 w "sensownych" danych oraz alokacje w seti'ie pamieci "na zapas".
Ostatecznie `dijkstra_speed` okazal sie minimalnie najlepszym algorytmem.
Zmierzony czas na pliku 2000 x 2000, to ok 23s i zurzycie pamieci 50MB.

Sa 2 interface'y obliczenia rozwiazania:
```rust
fn solve(&self, with_path: bool) -> Option<(DimType, VecDeque<NodeAdr>)>
```
oraz:
```rust
fn solve_and_drop(self, with_path: bool) -> Option<(DimType, VecDeque<NodeAdr>)>
```

Pierwsza przydaje sie w sytuacji kiedy chcemy iteracyjnie pracowac na tym samym zbiorze.
Ta druga daje programiscie wieksze mozliwości recznej optymalizacji pamięciowej.

Do zrobienia
-------------
Czas na zadanie był ograniczone (pare dni), dlatego jest kilka rzeczy to możliwego ulepszenia:

- Oznaczone w kodzie jako TODO.
- Dokumentacja kodu w formacie rustdoc
- Wiecej unit testow i testow integracyjnych.
- Testy wydajności (`cargo bench`)
- Bardziej formalny dowód poprawnosci.
- Ograniczenie (zastapienie czyms lepszym) dosc duzej liczby rzutowań `as`.


License
--------
MIT-like. Derivative work is possible, but it must reference the source.

Author
--------
Grzegorz Wierzchowski
gwierzchowski@wp.pl


