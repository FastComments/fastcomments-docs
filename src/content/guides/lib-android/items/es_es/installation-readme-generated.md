Agrega el SDK de FastComments al archivo `build.gradle.kts` de tu aplicación:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

Asegúrate de tener el repositorio Repsy en el archivo `settings.gradle.kts` de tu proyecto:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // other repositories...
    }
}
```