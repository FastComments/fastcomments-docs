### Maven

Tilføj Repsy repository til dit projekts POM:

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
        <version>1.3.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.2</version>
    </dependency>
</dependencies>
```

### Gradle

Tilføj Repsy repository til din build.gradle-fil:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:1.3.2"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:1.3.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:1.3.2"
}
```

### Biblioteksindhold

Dette bibliotek indeholder tre moduler. Den genererede API-klient, det centrale Java-bibliotek som indeholder håndskrevne hjælpefunktioner til at gøre arbejdet med API'et nemmere, og `pubsub` modulet som er et bibliotek til at abonnere på ændringsfeeds.

- [Dokumentation for API-klientbiblioteket](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentation for Core-biblioteket, inklusive SSO-eksempler](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentation for PubSub-biblioteket](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Offentlige vs. Sikrede API'er

For API-klienten findes der to klasser, `DefaultApi` og `PublicApi`. `DefaultApi` indeholder metoder, der kræver din API-nøgle, og `PublicApi` indeholder API-kald, som kan foretages direkte fra en browser/mobile enhed/etc uden autentificering.