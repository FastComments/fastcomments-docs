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
    <!-- Klijent API-ja -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- Core knjižnica (uključuje SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- PubSub knjižnica (za događaje uživo) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.2</version>
    </dependency>
</dependencies>
```

### Gradle

Dodajte Repsy repozitorij u datoteku build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // Klijent API-ja
    implementation "com.fastcomments:client:1.3.2"
    
    // Core knjižnica (uključuje SSO)
    implementation "com.fastcomments:core:1.3.2"
    
    // PubSub knjižnica (za događaje uživo)
    implementation "com.fastcomments:pubsub:1.3.2"
}
```

### Sadržaj knjižnice

Ova knjižnica sadrži tri modula. Generirani API klijent, osnovna Java knjižnica koja sadrži ručno napisane pomoćne funkcije za olakšavanje rada s API-jem, i modul `pubsub` koji je knjižnica za pretplatu na feed promjena.

- [Dokumentacija API klijenta](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacija Core knjižnice, uključujući SSO primjere](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacija PubSub knjižnice](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Javni vs Zaštićeni API-ji

Za API klijent postoje dvije klase, `DefaultApi` i `PublicApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, a `PublicApi` sadrži API pozive koji se mogu izvršiti izravno iz preglednika/mobilnog uređaja/itd. bez autentifikacije.