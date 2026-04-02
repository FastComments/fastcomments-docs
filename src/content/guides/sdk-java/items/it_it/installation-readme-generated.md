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
    <!-- Client API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- Libreria Core (include SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- Libreria PubSub (per eventi in tempo reale) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.2</version>
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
    // Client API
    implementation "com.fastcomments:client:1.3.2"
    
    // Libreria Core (include SSO)
    implementation "com.fastcomments:core:1.3.2"
    
    // Libreria PubSub (per eventi in tempo reale)
    implementation "com.fastcomments:pubsub:1.3.2"
}
```

### Library Contents

Questa libreria contiene tre moduli. Il client API generato, la libreria Java core che contiene utility scritte a mano per rendere più semplice il lavoro con l'API, e il modulo `pubsub` che è una libreria per iscriversi ai feed di modifica.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Per il client API, ci sono due classi, `DefaultApi` e `PublicApi`. `DefaultApi` contiene i metodi che richiedono la tua API key, e `PublicApi` contiene chiamate API che possono essere fatte direttamente da un browser/dispositivo mobile/etc senza autenticazione.