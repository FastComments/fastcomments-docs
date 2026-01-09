Teraz, gdy pobraliśmy plik zip, rozpakuj go do folderu. Pobrałem domyślny `casper.zip` i rozpakowałem go do `Downloads\casper` na systemie Windows.

Następnie upewnij się, że masz zainstalowaną wersję LTS lub nowszą NodeJS. Możesz ją pobrać tutaj: https://nodejs.org/en/download/

Po zainstalowaniu NodeJS zainstaluj edytor kodu.

Polecamy (i używamy) Webstorm, który możesz pobrać tutaj z 30-dniową wersją próbną (nie jest potrzebna karta kredytowa): https://www.jetbrains.com/webstorm/

Kolejną najlepszą darmową opcją prawdopodobnie jest Visual Studio Code: https://code.visualstudio.com/download

Gdy skonfigurujesz edytor i otworzysz folder motywu w edytorze, otwórz terminal w IDE i uruchom:

[inline-code-attrs-start title = 'Zainstaluj motyw'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

Pomyślny wynik będzie wyglądać tak (możesz zignorować ostrzeżenia):

<div class="screenshot white-bg">
    <div class="title">Pomyślny wynik npm install</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Pomyślny wynik npm install" />
</div>

To skonfiguruje zależności motywu dla poleceń, które uruchomimy później. Eksport zależy również od zainstalowania zależności motywu; w przeciwnym razie ponowny import nie będzie działać prawidłowo.

---