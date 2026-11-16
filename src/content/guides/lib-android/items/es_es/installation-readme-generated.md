Añade el SDK de FastComments al archivo `build.gradle.kts` de tu aplicación:

```kotlin
dependencies {
    implementation("com.fastcomments:android-sdk:2.1.0")
}
```

Asegúrate de tener el repositorio de Repsy en el archivo `settings.gradle.kts` de tu proyecto:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // otros repositorios...
    }
}
```