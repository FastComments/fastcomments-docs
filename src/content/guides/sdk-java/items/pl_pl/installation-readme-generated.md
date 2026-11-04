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
    <!-- Klient API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- Biblioteka Core (zawiera SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- Biblioteka PubSub (dla zdarzeń na żywo) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.1</version>
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
    // Klient API
    implementation "com.fastcomments:client:3.1.1"
    
    // Biblioteka Core (zawiera SSO)
    implementation "com.fastcomments:core:3.1.1"
    
    // Biblioteka PubSub (dla zdarzeń na żywo)
    implementation "com.fastcomments:pubsub:3.1.1"
}
```

### Zawartość biblioteki

Ta biblioteka zawiera trzy moduły: wygenerowanego klienta API, podstawową bibliotekę Java zawierającą ręcznie pisane narzędzia
ułatwiające pracę z API oraz moduł `pubsub`, który jest biblioteką do subskrybowania kanałów zmian.

- [Dokumentacja biblioteki klienta API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacja biblioteki Core, w tym przykłady SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacja biblioteki PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### API publiczne vs zabezpieczone

W przypadku klienta API dostępne są trzy klasy: `DefaultApi`, `PublicApi` i `ModerationApi`. Klasa `DefaultApi` zawiera metody wymagające klucza API, a `PublicApi` zawiera metody,
które mogą być wywoływane bezpośrednio z przeglądarki/urządzenia mobilnego itp. bez uwierzytelniania.

Klasa `ModerationApi` zapewnia rozbudowany zestaw szybkich API do moderacji na żywo. Każda metoda `ModerationApi` przyjmuje parametr `sso` i może uwierzytelniać za pomocą SSO lub ciasteczka sesji FastComments.com.