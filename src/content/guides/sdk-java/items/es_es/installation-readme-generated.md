### Maven

Añade el repositorio Repsy al POM de tu proyecto:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Luego añade las dependencias que necesites:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Añade el repositorio Repsy a tu archivo **build.gradle**:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:3.0.0"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.0.0"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.0.0"
}
```

### Contenido de la biblioteca

Esta biblioteca contiene tres módulos. El cliente API generado, la biblioteca Java central que contiene utilidades escritas a mano para facilitar el trabajo con la API, y el módulo `pubsub`, que es una biblioteca para suscribirse a flujos de cambios.

- [Documentación de la biblioteca del cliente API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentación de la biblioteca principal, incluidos ejemplos de SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentación de la biblioteca PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### APIs públicas vs. seguras

Para el cliente API, existen tres clases: `DefaultApi`, `PublicApi` y `ModerationApi`. La `DefaultApi` contiene métodos que requieren tu clave API, y la `PublicApi` contiene métodos que pueden ejecutarse directamente desde un navegador/dispositivo móvil, etc., sin autenticación.

La `ModerationApi` ofrece una amplia suite de APIs de moderación en tiempo real y de alta velocidad. Cada método de `ModerationApi` acepta un parámetro `sso` y puede autenticarse mediante SSO o una cookie de sesión de FastComments.com.