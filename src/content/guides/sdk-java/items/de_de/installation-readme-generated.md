### Maven

Fügen Sie das Repsy-Repository zur POM Ihres Projekts hinzu:

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
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.1</version>
    </dependency>
</dependencies>
```

### Gradle

Fügen Sie das Repsy-Repository zu Ihrer build.gradle-Datei hinzu:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:3.1.1"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.1.1"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.1.1"
}
```

### Inhalt der Bibliothek

Diese Bibliothek enthält drei Module: den generierten API-Client, die Core-Java-Bibliothek mit handgeschriebenen Hilfsfunktionen
zur Vereinfachung der Arbeit mit der API sowie das `pubsub`-Modul, eine Bibliothek zum Abonnieren von Change-Feeds.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Öffentliche vs. gesicherte APIs

Für den API-Client gibt es drei Klassen: `DefaultApi`, `PublicApi` und `ModerationApi`. Die `DefaultApi` enthält Methoden, die Ihren API-Schlüssel benötigen, und die `PublicApi` enthält Methoden,
die direkt aus einem Browser/Mobilgerät/etc. ohne Authentifizierung aufgerufen werden können.

Die `ModerationApi` bietet eine umfangreiche Suite an Live- und schnellen Moderations-APIs. Jede `ModerationApi`-Methode akzeptiert einen `sso`-Parameter und kann sich über SSO oder ein FastComments.com-Session-Cookie authentifizieren.