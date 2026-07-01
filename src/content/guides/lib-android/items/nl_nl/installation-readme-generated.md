Add the FastComments SDK toe aan je app’s `build.gradle.kts` bestand:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:2.0.0")
}
```

Zorg ervoor dat je de Repsy repository in het `settings.gradle.kts` bestand van je project hebt:

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