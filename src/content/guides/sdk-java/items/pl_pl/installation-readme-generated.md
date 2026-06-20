### Maven

Dodaj repozytorium Repsy do pliku POM swojego projektu:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Następnie dodaj wymagane zależności:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Dodaj repozytorium Repsy do pliku build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:2.0.0"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Library Contents

Ta biblioteka zawiera trzy moduły. Wygenerowany klient API, biblioteka core w Javie, która zawiera ręcznie napisane narzędzia ułatwiające pracę z API, oraz moduł `pubsub`, który jest biblioteką do subskrybowania strumieni zmian.

- [Dokumentacja biblioteki klienta API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacja biblioteki Core, w tym przykłady SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacja biblioteki PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Dla klienta API istnieją trzy klasy: `DefaultApi`, `PublicApi` oraz `ModerationApi`. `DefaultApi` zawiera metody, które wymagają Twojego klucza API, a `PublicApi` zawiera metody, które można wywołać bezpośrednio z przeglądarki/urządzenia mobilnego itp. bez uwierzytelniania.

`ModerationApi` zasila panel moderatora. Zawiera metody do moderacji komentarzy (lista, zliczanie, wyszukiwanie, logi i eksport), akcje moderacji (usuwanie/przywracanie, oznaczanie, ustawianie statusu do weryfikacji/spam/akceptacja, głosowanie oraz ponowne otwieranie/zamykanie wątku), blokady (zablokowanie komentowania, cofnięcie blokady, podsumowania przed blokadą, status i preferencje blokady oraz liczba zablokowanych użytkowników) oraz odznaki i zaufanie (przyznawanie/usuwanie odznaki, odznaki ręczne, pobieranie/ustawianie współczynnika zaufania oraz wewnętrzny profil użytkownika). Każda metoda `ModerationApi` przyjmuje parametr `sso`, dzięki czemu wywołanie może zostać wykonane w imieniu moderatora uwierzytelnionego przez SSO.