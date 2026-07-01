### Maven

Aggiungi il repository Repsy al POM del tuo progetto:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Quindi aggiungi le dipendenze di cui hai bisogno:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Aggiungi il repository Repsy al file build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:2.0.0"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Contenuto della libreria

Questa libreria contiene tre moduli. Il client API generato, la libreria core Java che contiene utilità scritte a mano per semplificare l'uso dell'API, e il modulo `pubsub` che è una libreria per sottoscrivere i feed di cambiamento.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### API pubbliche vs API protette

Per il client API, ci sono tre classi, `DefaultApi`, `PublicApi` e `ModerationApi`. La classe `DefaultApi` contiene metodi che richiedono la tua chiave API, mentre `PublicApi` contiene metodi che possono essere chiamati direttamente da un browser/dispositivo mobile/etc. senza autenticazione.

`ModerationApi` offre una suite completa di API di moderazione in tempo reale e veloce. Ogni metodo di `ModerationApi` accetta un parametro `sso` e può autenticarsi tramite SSO o un cookie di sessione FastComments.com.