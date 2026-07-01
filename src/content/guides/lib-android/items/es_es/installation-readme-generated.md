Agrega el SDK de FastComments al archivo `build.gradle.kts` de tu aplicación:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:2.0.0")
}
```

Asegúrate de que tengas el repositorio Repsy en el `settings.gradle.kts` de tu proyecto:

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