### Maven

Voeg de Repsy-repository toe aan de POM van uw project:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Voeg vervolgens de benodigde afhankelijkheden toe:

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

Voeg de Repsy-repository toe aan uw build.gradle-bestand:

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

### Inhoud van de bibliotheek

Deze bibliotheek bevat drie modules. De gegenereerde API-client, de core Java-bibliotheek die handgeschreven hulpprogramma's bevat om het werken met de API te vergemakkelijken, en de `pubsub`-module die een bibliotheek is om u te abonneren op wijzigingsfeeds.

- [Documentatie API-clientbibliotheek](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentatie Core-bibliotheek, inclusief SSO-voorbeelden](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentatie PubSub-bibliotheek](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Publieke versus beveiligde API's

Voor de API-client zijn er twee klassen, `DefaultApi` en `PublicApi`. De `DefaultApi` bevat methoden die uw API-sleutel vereisen, en `PublicApi` bevat API-aanroepen die rechtstreeks vanuit een browser/mobiel apparaat/etc. kunnen worden gedaan zonder authenticatie.