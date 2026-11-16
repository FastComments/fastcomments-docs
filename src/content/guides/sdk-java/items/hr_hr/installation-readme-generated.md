### Maven

Dodajte Repsy repozitorij u POM datoteku svog projekta:

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
        <version>3.1.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.2</version>
    </dependency>
</dependencies>
```

### Gradle

Dodajte Repsy repozitorij u svoju build.gradle datoteku:

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
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.1.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.1.2"
}
```

### Sadržaj biblioteke

Ova biblioteka sadrži tri modula: generirani API klijent, core Java biblioteku koja sadrži ručno pisane uslužne programe
za lakši rad s API-jem, te `pubsub` modul koji je biblioteka za pretplatu na feedove promjena.

- [Dokumentacija API klijent biblioteke](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacija core biblioteke, uključujući SSO primjere](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacija PubSub biblioteke](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Javni naspram osiguranih API-ja

Za API klijent postoje tri klase: `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, a `PublicApi` sadrži metode
koje se mogu izvršavati izravno iz preglednika/mobilnog uređaja itd. bez autentifikacije.

`ModerationApi` pruža opsežan paket API-ja za moderiranje u stvarnom vremenu. Svaka metoda `ModerationApi` prihvaća `sso` parametar i može se autentificirati putem SSO-a ili FastComments.com session kolačića.