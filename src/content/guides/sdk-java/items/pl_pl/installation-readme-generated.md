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

Następnie dodaj potrzebne zależności:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.1</version>
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
    implementation "com.fastcomments:client:3.0.1"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.0.1"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.0.1"
}
```

### Library Contents

Ta biblioteka zawiera trzy moduły. Wygenerowany klient API, podstawowa biblioteka Java, która zawiera ręcznie napisane narzędzia ułatwiające pracę z API, oraz moduł `pubsub`, będący biblioteką do subskrybowania strumieni zmian.

- [Dokumentacja biblioteki klienta API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacja biblioteki podstawowej, w tym przykłady SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacja biblioteki PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Dla klienta API istnieją trzy klasy: `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` zawiera metody wymagające klucza API, natomiast `PublicApi` zawiera metody, które można wywołać bezpośrednio z przeglądarki, urządzenia mobilnego itp., bez uwierzytelnienia.

`ModerationApi` oferuje rozbudowany zestaw szybkich i bieżących API moderacji. Każda metoda `ModerationApi` przyjmuje parametr `sso` i może uwierzytelnić się za pomocą SSO lub ciasteczka sesji FastComments.com.