### Maven

Tilføj Repsy-repositoriet til dit projekts POM:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Tilføj derefter de afhængigheder, du har brug for:

```xml
<dependencies>
    <!-- API-klient -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- Kernebibliotek (inkluderer SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- PubSub-bibliotek (til live‑begivenheder) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.1</version>
    </dependency>
</dependencies>
```

### Gradle

Tilføj Repsy-repositoriet til din `build.gradle`‑fil:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API-klient
    implementation "com.fastcomments:client:3.0.1"
    
    // Kernebibliotek (inkluderer SSO)
    implementation "com.fastcomments:core:3.0.1"
    
    // PubSub-bibliotek (til live‑begivenheder)
    implementation "com.fastcomments:pubsub:3.0.1"
}
```

### Biblioteksindhold

Dette bibliotek indeholder tre moduler: den genererede API‑klient, kernebiblioteket for Java som indeholder håndskrevne hjælpefunktioner for at gøre arbejdet med API‑et lettere, samt `pubsub`‑modulet som er et bibliotek til at abonnere på ændringsfeeds.

- [API‑klientbibliotekets dokumentation](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Kernebibliotekets dokumentation, inklusiv SSO‑eksempler](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub‑bibliotekets dokumentation](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Offentlige vs sikrede API'er

For API‑klienten findes der tre klasser, `DefaultApi`, `PublicApi` og `ModerationApi`. `DefaultApi` indeholder metoder, der kræver din API‑nøgle, mens `PublicApi` indeholder metoder, der kan kaldes direkte fra en browser/mobil enhed osv. uden autentificering.

`ModerationApi` leverer et omfattende sæt af live‑ og hurtige moderations‑API’er. Hver `ModerationApi`‑metode accepterer en `sso`‑parameter og kan autentificeres via SSO eller en FastComments.com‑sessions‑cookie.