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

Tilføj Repsy-repositoriet til din build.gradle-fil:

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

### Biblioteksindhold

Dette bibliotek indeholder tre moduler. Den genererede API-klient, kernebiblioteket i Java som indeholder håndskrevne værktøjer for at gøre arbejdet med API'en lettere, og `pubsub`-modulet som er et bibliotek til at abonnere på ændringsfeeds.

- [API-klientbibliotekets dokumentation](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Kernebibliotekets dokumentation, inklusiv SSO-eksempler](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub-bibliotekets dokumentation](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Offentlige vs sikrede API'er

For API-klienten findes der tre klasser, `DefaultApi`, `PublicApi` og `ModerationApi`. `DefaultApi` indeholder metoder, der kræver din API-nøgle, og `PublicApi` indeholder metoder, som kan kaldes direkte fra en browser/mobil enhed osv. uden godkendelse.

`ModerationApi` leverer en omfattende række af live og hurtige moderations-API'er. Hver `ModerationApi`-metode accepterer en `sso`-parameter og kan autentificere via SSO eller en FastComments.com-session cookie.