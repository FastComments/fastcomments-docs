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
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- Core Library (uključuje SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- PubSub Library (za live eventove) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.2</version>
    </dependency>
</dependencies>
```

### Gradle

Dodajte Repsy repozitorijum u svoju build.gradle datoteku:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:3.1.2"
    
    // Core Library (uključuje SSO)
    implementation "com.fastcomments:core:3.1.2"
    
    // PubSub Library (za live eventove)
    implementation "com.fastcomments:pubsub:3.1.2"
}
```

### Sadržaj biblioteke

Ova biblioteka sadrži tri modula. Generisani API klijent, core Java biblioteku koja sadrži ručno pisane alate
za lakši rad sa API-jem, i `pubsub` modul koji je biblioteka za pretplatu na feedove promena.

- [Dokumentacija API klijent biblioteke](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacija Core biblioteke, uključujući SSO primere](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacija PubSub biblioteke](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Javni naspram zaštićenih API-ja

Za API klijent postoje tri klase: `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtevaju vaš API ključ, a `PublicApi` sadrži metode
koje se mogu pozivati direktno iz brauzera/mobilnog uređaja itd. bez autentifikacije.

`ModerationApi` pruža opsežan paket live i brzih API-ja za moderaciju. Svaka metoda `ModerationApi` prihvata `sso` parametar i može se autentifikovati putem SSO-a ili FastComments.com session kolačića.