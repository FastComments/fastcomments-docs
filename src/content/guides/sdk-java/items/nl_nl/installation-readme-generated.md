### Maven

Voeg de Repsy-repository toe aan de POM van je project:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Voeg vervolgens de dependencies toe die je nodig hebt:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.2</version>
    </dependency>
</dependencies>
```

### Gradle

Voeg de Repsy-repository toe aan je build.gradle-bestand:

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
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.1.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.1.2"
}
```

### Inhoud van de bibliotheek

Deze bibliotheek bevat drie modules: de gegenereerde API-client, de core Java-bibliotheek die handgeschreven hulpprogramma's bevat om het werken met de API eenvoudiger te maken, en de `pubsub`-module, een bibliotheek voor het abonneren op change feeds.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Publieke versus beveiligde API's

Voor de API-client zijn er drie klassen: `DefaultApi`, `PublicApi` en `ModerationApi`. De `DefaultApi` bevat methoden die je API-sleutel vereisen, en `PublicApi` bevat methoden
die direct vanuit een browser/mobiel apparaat/enz. kunnen worden aangeroepen zonder authenticatie.

De `ModerationApi` biedt een uitgebreide reeks live en snelle moderatie-API's. Elke `ModerationApi`-methode accepteert een `sso`-parameter en kan authenticeren via SSO of een FastComments.com-sessiecookie.