### Maven

Fügen Sie das Repsy-Repository zu Ihrer Projekt-POM hinzu:

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
        <version>3.0.1</version>
    </dependency>
    
    <!-- Core-Bibliothek (enthält SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- PubSub-Bibliothek (für Live-Ereignisse) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.1</version>
    </dependency>
</dependencies>
```

### Gradle

Fügen Sie das Repsy-Repository zu Ihrer `build.gradle`‑Datei hinzu:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API-Client
    implementation "com.fastcomments:client:3.0.1"
    
    // Core-Bibliothek (enthält SSO)
    implementation "com.fastcomments:core:3.0.1"
    
    // PubSub-Bibliothek (für Live-Ereignisse)
    implementation "com.fastcomments:pubsub:3.0.1"
}
```

### Bibliotheksinhalte

Diese Bibliothek enthält drei Module: den generierten API-Client, die Kern‑Java‑Bibliothek, die handgeschriebene Hilfsprogramme enthält, um die Arbeit mit der API zu erleichtern, und das `pubsub`‑Modul, das eine Bibliothek zum Abonnieren von Änderungsfeeds ist.

- [API-Client-Bibliotheksdokumentation](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core-Bibliotheksdokumentation, einschließlich SSO-Beispielen](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub-Bibliotheksdokumentation](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Öffentliche vs gesicherte APIs

Für den API-Client gibt es drei Klassen, `DefaultApi`, `PublicApi` und `ModerationApi`. `DefaultApi` enthält Methoden, die Ihren API-Schlüssel erfordern, und `PublicApi` enthält Methoden, die direkt von einem Browser/Mobilgerät usw. ohne Authentifizierung aufgerufen werden können.

`ModerationApi` bietet eine reiche Palette von Live- und schnellen Moderations-APIs. Jede `ModerationApi`‑Methode akzeptiert einen `sso`‑Parameter und kann sich über SSO oder ein FastComments.com‑Sitzungs‑Cookie authentifizieren.