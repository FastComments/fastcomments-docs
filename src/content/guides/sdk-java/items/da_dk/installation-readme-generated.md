### Maven

Tilføj Repsy-repositoryet til dit projekts POM:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Tilføj derefter de dependencies, du har brug for:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- Core Library (inkluderer SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- PubSub Library (til live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.2</version>
    </dependency>
</dependencies>
```

### Gradle

Tilføj Repsy-repositoryet til din build.gradle-fil:

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
    
    // Core Library (inkluderer SSO)
    implementation "com.fastcomments:core:3.1.2"
    
    // PubSub Library (til live events)
    implementation "com.fastcomments:pubsub:3.1.2"
}
```

### Bibliotekets indhold

Dette bibliotek indeholder tre moduler. Den genererede API-klient, core Java-biblioteket, som indeholder håndskrevne utilities
der gør det lettere at arbejde med API'et, samt `pubsub`-modulet, som er et bibliotek til at abonnere på change feeds.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, inklusive SSO-eksempler](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Offentlige vs. sikrede API'er

For API-klienten er der tre klasser: `DefaultApi`, `PublicApi` og `ModerationApi`. `DefaultApi` indeholder metoder, der kræver din API-nøgle, og `PublicApi` indeholder metoder,
der kan kaldes direkte fra en browser/mobil enhed/osv. uden autentificering.

`ModerationApi` giver en omfattende pakke af live- og hurtige moderations-API'er. Hver `ModerationApi`-metode accepterer en `sso`-parameter og kan autentificere via SSO eller en FastComments.com-session-cookie.