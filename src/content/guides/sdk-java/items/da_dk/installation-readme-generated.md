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

Then add the dependencies you need:

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
    implementation "com.fastcomments:client:0.0.2"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:0.0.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:0.0.2"
}
```

### Bibliotekets indhold

Dette bibliotek indeholder tre moduler. Den genererede API-klient, kerne-Java-biblioteket som indeholder håndskrevne værktøjer
til at gøre arbejdet med API'et lettere, og `pubsub`-modulet som er et bibliotek til at abonnere på ændringsfeeds.

- [Dokumentation for API-klientbiblioteket](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentation for kernebiblioteket, inklusive SSO-eksempler](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentation for PubSub-biblioteket](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Offentlige vs. Sikrede API'er

For API-klienten er der to klasser, `DefaultApi` og `PublicApi`. `DefaultApi` indeholder metoder, der kræver din API-nøgle, og `PublicApi` indeholder API-opkald, der kan foretages direkte fra en browser, mobil enhed osv. uden godkendelse.