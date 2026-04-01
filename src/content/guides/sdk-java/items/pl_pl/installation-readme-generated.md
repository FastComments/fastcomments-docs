### Maven

Dodaj repozytorium Repsy do pliku POM projektu:

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
        <version>1.3.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.2</version>
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
    implementation "com.fastcomments:client:1.3.2"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:1.3.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:1.3.2"
}
```

### Zawartość biblioteki

Ta biblioteka zawiera trzy moduły. Wygenerowanego klienta API, bibliotekę Core w Javie, która zawiera ręcznie napisane narzędzia ułatwiające pracę z API, oraz moduł `pubsub`, który jest biblioteką do subskrybowania kanałów zmian.

- [Dokumentacja biblioteki klienta API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacja biblioteki Core, w tym przykłady SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacja biblioteki PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Publiczne vs Zabezpieczone API

Dla klienta API dostępne są dwie klasy: `DefaultApi` i `PublicApi`. `DefaultApi` zawiera metody, które wymagają twojego klucza API, a `PublicApi` zawiera wywołania API, które można wykonywać bezpośrednio z przeglądarki/urządzenia mobilnego itp. bez uwierzytelniania.