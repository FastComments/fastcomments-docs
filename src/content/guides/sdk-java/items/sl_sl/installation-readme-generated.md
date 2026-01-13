### Maven

Dodajte Repsy repozitorij v POM datoteko vašega projekta:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Nato dodajte odvisnosti, ki jih potrebujete:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>0.0.2</version>
    </dependency>
</dependencies>
```

### Gradle

Dodajte Repsy repozitorij v datoteko build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:0.0.2"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:0.0.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:0.0.2"
}
```

### Vsebina knjižnice

Ta knjižnica vsebuje tri module. Generirani API odjemalec, osnovna Java knjižnica, ki vsebuje ročno napisane pripomočke za lažje delo z API-jem, in modul `pubsub`, ki je knjižnica za naročanje na tokove sprememb.

- [Dokumentacija knjižnice API odjemalca](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacija osnovne knjižnice, vključno s primeri SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacija PubSub knjižnice](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Javni in zavarovani API-ji

Za API odjemalca obstajata dva razreda, `DefaultApi` in `PublicApi`. `DefaultApi` vsebuje metode, ki zahtevajo vaš API ključ, medtem ko `PublicApi` vsebuje klice API-ja, ki jih lahko naredite neposredno iz brskalnika/mobilne naprave/ipd. brez avtentikacije.