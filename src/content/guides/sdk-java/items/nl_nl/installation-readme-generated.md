### Maven

Voeg de Repsy-repository toe aan het POM-bestand van uw project:

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
    <!-- API-client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- Core-bibliotheek (inclusief SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- PubSub-bibliotheek (voor live gebeurtenissen) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.1</version>
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
    // API-client
    implementation "com.fastcomments:client:1.3.1"
    
    // Core-bibliotheek (inclusief SSO)
    implementation "com.fastcomments:core:1.3.1"
    
    // PubSub-bibliotheek (voor live gebeurtenissen)
    implementation "com.fastcomments:pubsub:1.3.1"
}
```

### Inhoud van de bibliotheek

Deze bibliotheek bevat drie modules. De gegenereerde API-client, de core Java-bibliotheek die handgeschreven hulpmiddelen bevat om het werken met de API te vereenvoudigen, en de `pubsub` module die een bibliotheek is om zich te abonneren op wijzigingsfeeds.

- [Documentatie API-clientbibliotheek](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentatie Core-bibliotheek, inclusief SSO-voorbeelden](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentatie PubSub-bibliotheek](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Publieke vs beveiligde API's

Voor de API-client zijn er twee klassen, `DefaultApi` en `PublicApi`. De `DefaultApi` bevat methoden die uw API-sleutel vereisen, en `PublicApi` bevat API-aanroepen die rechtstreeks vanuit een browser, een mobiel apparaat, enz. kunnen worden uitgevoerd zonder authenticatie.