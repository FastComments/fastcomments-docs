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

Zatim dodajte ovisnosti koje su vam potrebne:

```xml
<dependencies>
    <!-- Klijent API-ja -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- Core biblioteka (uključuje SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- PubSub biblioteka (za događaje u stvarnom vremenu) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.1</version>
    </dependency>
</dependencies>
```

### Gradle

Dodajte Repsy repozitorij u vaš build.gradle file:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // Klijent API-ja
    implementation "com.fastcomments:client:3.0.1"
    
    // Core biblioteka (uključuje SSO)
    implementation "com.fastcomments:core:3.0.1"
    
    // PubSub biblioteka (za događaje u stvarnom vremenu)
    implementation "com.fastcomments:pubsub:3.0.1"
}
```

### Sadržaj biblioteke

Biblioteka sadrži tri modula. Generirani API klijent, core Java biblioteka koja sadrži ručno napisane alate za olakšavanje rada s API-jem, i `pubsub` modul koji je biblioteka za pretplatu na promjene feedova.

- [Dokumentacija API klijentske biblioteke](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacija core biblioteke, uključujući SSO primjere](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacija PubSub biblioteke](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Javni vs Zaštićeni API-ji

Za API klijent postoje tri klase, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, a `PublicApi` sadrži metode koje se mogu pozvati izravno iz preglednika/mobilnog uređaja itd. bez autentifikacije.

`ModerationApi` pruža opsežan skup live i brzih moderacijskih API-ja. Svaka metoda `ModerationApi` prihvaća `sso` parametar i može se autentificirati putem SSO-a ili putem FastComments.com sesijskog kolačića.