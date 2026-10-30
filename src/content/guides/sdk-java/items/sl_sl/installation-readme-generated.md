### Maven

Dodajte Repsy repozitorij v POM vašega projekta:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Nato dodajte potrebne odvisnosti:

```xml
<dependencies>
    <!-- API odjemalec -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- Jedrni knjižnici (vključuje SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- PubSub knjižnica (za žive dogodke) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.1</version>
    </dependency>
</dependencies>
```

### Gradle

Dodajte Repsy repozitorij v datoteko **build.gradle**:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API odjemalec
    implementation "com.fastcomments:client:3.1.1"
    
    // Jedrni knjižnici (vključuje SSO)
    implementation "com.fastcomments:core:3.1.1"
    
    // PubSub knjižnica (za žive dogodke)
    implementation "com.fastcomments:pubsub:3.1.1"
}
```

### Vsebina knjižnice

Ta knjižnica vsebuje tri module: ustvarjenega API odjemalca, jedrno Java knjižnico, ki vsebuje ročno napisane pripomočke za lažje delo z API-jem, in modul `pubsub`, ki je knjižnica za naročanje na spremembe.

- [Dokumentacija knjižnice API odjemalca](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacija jedrne knjižnice, vključno s primeri SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacija PubSub knjižnice](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Javni vs. zaščiteni API-ji

Za API odjemalca obstajajo trije razredi: `DefaultApi`, `PublicApi` in `ModerationApi`. `DefaultApi` vsebuje metode, ki zahtevajo vaš API ključ, medtem ko `PublicApi` vsebuje metode, ki jih je mogoče klicati neposredno iz brskalnika/mobilne naprave itd. brez avtentikacije.

`ModerationApi` ponuja obsežen nabor živih in hitrih moderacijskih API-jev. Vsaka metoda `ModerationApi` sprejme parameter `sso` in se lahko avtenticira prek SSO ali piškotka seje FastComments.com.