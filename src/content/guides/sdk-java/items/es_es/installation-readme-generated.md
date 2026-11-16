### Maven

Añade el repositorio de Repsy al POM de tu proyecto:

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
    <!-- Cliente de API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- Biblioteca Core (incluye SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- Biblioteca PubSub (para eventos en vivo) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.2</version>
    </dependency>
</dependencies>
```

### Gradle

Añade el repositorio de Repsy a tu archivo build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // Cliente de API
    implementation "com.fastcomments:client:3.1.2"
    
    // Biblioteca Core (incluye SSO)
    implementation "com.fastcomments:core:3.1.2"
    
    // Biblioteca PubSub (para eventos en vivo)
    implementation "com.fastcomments:pubsub:3.1.2"
}
```

### Contenido de la Biblioteca

Esta biblioteca contiene tres módulos: el cliente de API generado, la biblioteca core de Java que contiene utilidades escritas a mano
para facilitar el trabajo con la API, y el módulo `pubsub`, que es una biblioteca para suscribirse a feeds de cambios.

- [Documentación de la Biblioteca de Cliente de API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentación de la Biblioteca Core, Incluyendo Ejemplos de SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentación de la Biblioteca PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### APIs Públicas vs Aseguradas

Para el cliente de API, hay tres clases: `DefaultApi`, `PublicApi` y `ModerationApi`. La `DefaultApi` contiene métodos que requieren tu clave de API, y la `PublicApi` contiene métodos
que pueden llamarse directamente desde un navegador/dispositivo móvil/etc. sin autenticación.

La `ModerationApi` ofrece un extenso conjunto de APIs de moderación en vivo y rápida. Cada método de `ModerationApi` acepta un parámetro `sso` y puede autenticarse mediante SSO o una cookie de sesión de FastComments.com.