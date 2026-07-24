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

Voeg vervolgens de benodigde dependencies toe:

```xml
<dependencies>
    <!-- API-client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- Core-bibliotheek (inclusief SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- PubSub-bibliotheek (voor live‑evenementen) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.1</version>
    </dependency>
</dependencies>
```

### Gradle

Voeg de Repsy-repository toe aan je `build.gradle`‑bestand:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API-client
    implementation "com.fastcomments:client:3.0.1"
    
    // Core-bibliotheek (inclusief SSO)
    implementation "com.fastcomments:core:3.0.1"
    
    // PubSub-bibliotheek (voor live‑evenementen)
    implementation "com.fastcomments:pubsub:3.0.1"
}
```

### Library Contents

Deze bibliotheek bevat drie modules: de gegenereerde API‑client, de core‑Java‑bibliotheek die handgeschreven hulpprogramma’s bevat om het werken met de API te vergemakkelijken, en de `pubsub`‑module, een bibliotheek voor het abonneren op wijzigingsfeeds.

- [API-clientbibliotheekdocumentatie](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core-bibliotheekdocumentatie, inclusief SSO‑voorbeelden](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSubscriptie‑bibliotheekdocumentatie](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Voor de API‑client zijn er drie klassen: `DefaultApi`, `PublicApi` en `ModerationApi`. `DefaultApi` bevat methoden die je API‑sleutel vereisen, en `PublicApi` bevat methoden die rechtstreeks vanuit een browser/mobiel apparaat/etc. kunnen worden aangeroepen zonder authenticatie.

`ModerationApi` biedt een uitgebreide reeks live‑ en snelle moderatie‑API’s. Elke `ModerationApi`‑methode accepteert een `sso`‑parameter en kan authenticeren via SSO of een FastComments.com‑sessiecookie.