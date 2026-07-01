### Maven

Fügen Sie das Repsy-Repository zu Ihrer Projekt‑POM hinzu:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Fügen Sie dann die benötigten Abhängigkeiten hinzu:

```xml
<dependencies>
    <!-- API-Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- Kernbibliothek (inkl. SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- PubSub‑Bibliothek (für Live‑Ereignisse) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Fügen Sie das Repsy-Repository zu Ihrer **build.gradle**‑Datei hinzu:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API-Client
    implementation "com.fastcomments:client:3.0.0"
    
    // Kernbibliothek (inkl. SSO)
    implementation "com.fastcomments:core:3.0.0"
    
    // PubSub‑Bibliothek (für Live‑Ereignisse)
    implementation "com.fastcomments:pubsub:3.0.0"
}
```

### Library Contents

Diese Bibliothek enthält drei Module: den generierten API‑Client, die Kern‑Java‑Bibliothek, die handgeschriebene Dienstprogramme zur vereinfachten Arbeit mit der API enthält, und das `pubsub`‑Modul, das eine Bibliothek zum Abonnieren von Änderungsfeeds bereitstellt.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Für den API‑Client gibt es drei Klassen, `DefaultApi`, `PublicApi` und `ModerationApi`. Die `DefaultApi` enthält Methoden, die Ihren API‑Schlüssel benötigen, und `PublicApi` enthält Methoden, die direkt aus einem Browser/Mobilgerät usw. ohne Authentifizierung aufgerufen werden können.

Die `ModerationApi` bietet eine umfangreiche Suite von Live‑ und schnellen Moderations‑APIs. Jede `ModerationApi`‑Methode akzeptiert einen `sso`‑Parameter und kann sich über SSO oder ein FastComments.com‑Session‑Cookie authentifizieren.