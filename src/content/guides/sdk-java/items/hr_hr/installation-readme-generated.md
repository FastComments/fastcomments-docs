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

Zatim dodajte potrebne ovisnosti:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Dodajte Repsy repozitorij u svoju datoteku `build.gradle`:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:3.0.0"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.0.0"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.0.0"
}
```

### Library Contents

Ova biblioteka sadrži tri modula. Generirani API klijent, osnovna Java biblioteka koja sadrži ručno napisane alate za olakšavanje rada s API‑jem, i `pubsub` modul koji je biblioteka za pretplatu na feedove promjena.

- [Dokumentacija API klijentske biblioteke](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacija osnovne biblioteke, uključujući SSO primjere](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacija PubSub biblioteke](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Za API klijenta postoje tri klase, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, a `PublicApi` sadrži metode koje se mogu pozvati izravno iz preglednika/mobilnog uređaja i sl. bez autentifikacije.

`ModerationApi` pruža opsežan skup API‑ja za brzu i live moderaciju. Svaka metoda `ModerationApi` prihvaća parametar `sso` i može se autentificirati putem SSO‑a ili sesijskog cookieja FastComments.com.