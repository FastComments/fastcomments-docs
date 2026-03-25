### Maven

Dodajte Repsy spremište u POM vašeg projekta:

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
    <!-- API klijent -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- Core biblioteka (uključuje SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- PubSub biblioteka (za događaje uživo) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.1</version>
    </dependency>
</dependencies>
```

### Gradle

Dodajte Repsy spremište u datoteku build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API klijent
    implementation "com.fastcomments:client:1.3.1"
    
    // Core biblioteka (uključuje SSO)
    implementation "com.fastcomments:core:1.3.1"
    
    // PubSub biblioteka (za događaje uživo)
    implementation "com.fastcomments:pubsub:1.3.1"
}
```

### Library Contents

Ova biblioteka sadrži tri modula. Generirani API klijent, core Java biblioteka koja sadrži ručno napisane pomoćne alate
koji olakšavaju rad s API-jem, i `pubsub` modul koji je biblioteka za pretplatu na feedove promjena.

- [Dokumentacija API klijentske biblioteke](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacija Core biblioteke, uključujući primjere SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacija PubSub biblioteke](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Za API klijenta postoje dvije klase, `DefaultApi` i `PublicApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, a `PublicApi` sadrži pozive API-ja koje je moguće izvršiti izravno iz preglednika/mobilnog uređaja/itd bez autentikacije.