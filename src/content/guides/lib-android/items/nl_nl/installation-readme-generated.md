Voeg de FastComments SDK toe aan het `build.gradle.kts`-bestand van uw app:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

Zorg ervoor dat u de Repsy-repository in het `settings.gradle.kts`-bestand van uw project hebt:

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