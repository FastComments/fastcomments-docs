### Maven

Agrega el repositorio Repsy al POM de tu proyecto:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Luego agrega las dependencias que necesites:

```xml
<dependencies>
    <!-- Cliente API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- Biblioteca Core (incluye SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- Biblioteca PubSub (para eventos en vivo) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.1</version>
    </dependency>
</dependencies>
```

### Gradle

Agrega el repositorio Repsy a tu archivo **build.gradle**:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // Cliente API
    implementation "com.fastcomments:client:3.1.1"
    
    // Biblioteca Core (incluye SSO)
    implementation "com.fastcomments:core:3.1.1"
    
    // Biblioteca PubSub (para eventos en vivo)
    implementation "com.fastcomments:pubsub:3.1.1"
}
```

### Contenido de la Biblioteca

Esta biblioteca contiene tres módulos: el cliente API generado, la biblioteca Java core que incluye utilidades escritas a mano para facilitar el trabajo con la API, y el módulo `pubsub`, que es una biblioteca para suscribirse a flujos de cambios.

- [Documentación de la Biblioteca Cliente API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentación de la Biblioteca Core, Incluyendo Ejemplos SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentación de la Biblioteca PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### APIs Públicas vs Seguras

Para el cliente API, existen tres clases: `DefaultApi`, `PublicApi` y `ModerationApi`. La `DefaultApi` contiene métodos que requieren tu clave API, y la `PublicApi` contiene métodos que pueden invocarse directamente desde un navegador/dispositivo móvil/etc. sin autenticación.

La `ModerationApi` ofrece una amplia gama de APIs de moderación en tiempo real y rápidas. Cada método de `ModerationApi` acepta un parámetro `sso` y puede autenticarse mediante SSO o una cookie de sesión de FastComments.com.