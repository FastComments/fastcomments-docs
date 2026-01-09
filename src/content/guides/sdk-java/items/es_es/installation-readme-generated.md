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

Then add the dependencies you need:

```xml
<dependencies>
    <!-- Cliente de la API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- Biblioteca principal (incluye SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- Biblioteca PubSub (para eventos en vivo) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>0.0.2</version>
    </dependency>
</dependencies>
```

### Gradle

Añade el repositorio Repsy a tu archivo build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // Cliente de la API
    implementation "com.fastcomments:client:0.0.2"
    
    // Biblioteca principal (incluye SSO)
    implementation "com.fastcomments:core:0.0.2"
    
    // Biblioteca PubSub (para eventos en vivo)
    implementation "com.fastcomments:pubsub:0.0.2"
}
```

### Contenido de la librería

Esta librería contiene tres módulos. El cliente de API generado, la librería Java principal que contiene utilidades escritas a mano para facilitar el trabajo con la API, y el módulo `pubsub`, que es una librería para suscribirse a flujos de cambios.

- [Documentación de la biblioteca del cliente de la API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentación de la biblioteca principal, incluidos ejemplos de SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentación de la biblioteca PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### APIs públicas frente a protegidas

Para el cliente de la API, hay dos clases, `DefaultApi` y `PublicApi`. `DefaultApi` contiene métodos que requieren tu clave de API, y `PublicApi` contiene llamadas a la API que se pueden realizar directamente desde un navegador, dispositivo móvil, etc., sin autenticación.