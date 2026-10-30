### Maven

Add the Repsy repository to your project's POM:

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
    <!-- API-client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- Core-bibliotheek (inclusief SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- PubSub-bibliotheek (voor live‑events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.1</version>
    </dependency>
</dependencies>
```

### Gradle

Add the Repsy repository to your build.gradle file:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API-client
    implementation "com.fastcomments:client:3.1.1"
    
    // Core-bibliotheek (inclusief SSO)
    implementation "com.fastcomments:core:3.1.1"
    
    // PubSub-bibliotheek (voor live‑events)
    implementation "com.fastcomments:pubsub:3.1.1"
}
```

### Library Contents

Deze bibliotheek bevat drie modules. De gegenereerde API‑client, de core‑Java‑bibliotheek die handgeschreven hulpprogramma’s bevat om het werken met de API makkelijker te maken, en de `pubsub`‑module die een bibliotheek is voor het abonneren op wijzigingsfeeds.

- [API‑clientbibliotheek‑documentatie](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core‑bibliotheek‑documentatie, inclusief SSO‑voorbeelden](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub‑bibliotheek‑documentatie](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Voor de API‑client zijn er drie klassen, `DefaultApi`, `PublicApi` en `ModerationApi`. De `DefaultApi` bevat methoden die je API‑sleutel vereisen, en de `PublicApi` bevat methoden die rechtstreeks vanuit een browser/mobiel apparaat/etc. kunnen worden aangeroepen zonder authenticatie.

De `ModerationApi` biedt een uitgebreide reeks live‑ en snelle moderatie‑API’s. Elke `ModerationApi`‑methode accepteert een `sso`‑parameter en kan authenticeren via SSO of een FastComments.com‑sessiecookie.