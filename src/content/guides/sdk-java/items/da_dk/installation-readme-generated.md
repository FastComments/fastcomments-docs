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

Tilføj derefter de afhængigheder, du har brug for:

```xml
<dependencies>
    <!-- API-klient -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- Core-bibliotek (inkluderer SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- PubSub-bibliotek (til live-begivenheder) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.1</version>
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
    // API-klient
    implementation "com.fastcomments:client:1.3.1"
    
    // Core-bibliotek (inkluderer SSO)
    implementation "com.fastcomments:core:1.3.1"
    
    // PubSub-bibliotek (til live-begivenheder)
    implementation "com.fastcomments:pubsub:1.3.1"
}
```

### Bibliotekets indhold

Dette bibliotek indeholder tre moduler. Den genererede API-klient, det centrale Java-bibliotek, som indeholder håndskrevne hjælpefunktioner for at gøre arbejdet med API'et nemmere, og `pubsub`-modulet, som er et bibliotek til at abonnere på ændringsfeeds.

- [API-klientbibliotekets dokumentation](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core-bibliotekets dokumentation, inklusive SSO-eksempler](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub-bibliotekets dokumentation](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Offentlige vs. sikrede API'er

For API-klienten findes der to klasser, `DefaultApi` og `PublicApi`. `DefaultApi` indeholder metoder, der kræver din API-nøgle, og `PublicApi` indeholder API-kald, der kan foretages direkte fra en browser/mobilenhed osv. uden autentificering.