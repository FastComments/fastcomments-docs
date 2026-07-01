### Maven

Dodajte Repsy repozitorijum u POM vašeg projekta:

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
    <!-- API Klijent -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Core Biblioteka (uključuje SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub Biblioteka (za live događaje) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Dodajte Repsy repozitorijum u vaš `build.gradle` fajl:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Klijent
    implementation "com.fastcomments:client:2.0.0"
    
    // Core Biblioteka (uključuje SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub Biblioteka (za live događaje)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Sadržaj Biblioteke

Ova biblioteka sadrži tri modula: generisani API klijent, core Java biblioteka koja sadrži ručno napisane alate za olakšavanje rada sa API‑jem, i `pubsub` modul koji je biblioteka za pretplatu na promjenske feedove.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Javni vs Zaštićeni API‑ji

Za API klijent postoje tri klase: `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, dok `PublicApi` sadrži metode koje se mogu pozvati direktno iz preglednika/ mobilnog uređaja itd. bez autentifikacije.

`ModerationApi` pruža opsežan set live i brzih moderacijskih API‑ja. Svaka metoda iz `ModerationApi` prima `sso` parametar i može se autentifikovati putem SSO‑a ili FastComments.com sesijskog kolačića.