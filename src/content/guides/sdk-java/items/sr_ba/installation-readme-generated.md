### Maven

Dodajte Repsy repozitorij u POM vašeg projekta:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Zatim dodajte zavisnosti koje su vam potrebne:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.1</version>
    </dependency>
</dependencies>
```

### Gradle

Dodajte Repsy repozitorij u vašu build.gradle datoteku:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:1.3.1"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:1.3.1"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:1.3.1"
}
```

### Sadržaj biblioteke

Ova biblioteka sadrži tri modula. Generisani API klijent, osnovna Java biblioteka koja sadrži ručno napisane utilitete za olakšavanje rada sa API-jem, i modul `pubsub` koji je biblioteka za pretplatu na feedove promjena.

- [Dokumentacija API klijenta](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacija Core biblioteke, uključujući primjere SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacija PubSub biblioteke](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Javni i Zaštićeni API-ji

Za API klijent postoje dvije klase, `DefaultApi` i `PublicApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, a `PublicApi` sadrži API pozive koji se mogu izvršiti direktno iz preglednika/mobilnog uređaja/itd bez autentifikacije.