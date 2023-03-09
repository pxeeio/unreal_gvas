# Gameplay Visual Analytics (GVAS)

GVAS is Unreal Engine's system that allows game developers to track and visualize player behavior and analytics data in real-time. This data is typically saved in a `.sav`. file for later analysis and visualization.


## Useful Links

### UE4.27 Documentation

- [Versioning of Assets and Packages](https://docs.unrealengine.com/4.27/en-US/ProgrammingAndScripting/ProgrammingWithCPP/UnrealArchitecture/VersioningAssetsAndPackages/)
- [FEngineVersionBase](https://docs.unrealengine.com/4.27/en-US/API/Runtime/Core/Misc/FEngineVersionBase/)
- [FString](https://docs.unrealengine.com/4.27/en-US/API/Runtime/Core/Containers/FString/)
    - [FString Handling](https://docs.unrealengine.com/4.27/en-US/ProgrammingAndScripting/ProgrammingWithCPP/UnrealArchitecture/StringHandling/FString/)


## File Header

### Structure

```
EngineVersion: [u8; ?] {
    [0:4]: File Signature
    [4:8]: Save Game File version
    [8:12]: Package File UE4 version
    [12:14]: UE Major version (u16)
    [14:16]: UE Minor version (u16)
    [16:18]: UE Patch version (u16)
    [18:22]: UE Changelist number (u32)
    [22:26]: UE Version Branch name char length (i32)
    [26:?]: UE Version Branch Name (FString)
}
```

Example `EngineVersion`: 4.19.0-3944462+++UE4+Release-4.19

- `Package version` refers to the version of the GVAS system itself, rather than a version of a specific package file.
    - The version number is used to indicate changes and updates to the GVAS system and the format of the `.sav` files it generates.
    - It's typically displayed in the GVAS Editor and other Unreal Engine tools that work with GVAS data.
- `Changelist` (`3944462`) is a number used to arbitrate when major/minor/patch version numbers match.
- `Branch name` (`+++UE4+Release-4.19`) is a string used to describe the branch from which the build was compiled.
    - It is used for display purposes or to uniquely identify a build.
    - Does not affect compatibility checks.

- `FString` is a type that represents a dynamically sizeable string
    - `FString::len()` in C++ would return an integer of type `i32`
    - `FString` is derived from the `UnrealString` type.


### Globally Unique Identifier (GUID)

In Unreal Engine, a GUID is a unique identifier used to identify various types of assets, such as textures, materials, meshes, blueprints, and more. 

It is a 128-bit number that is assigned to an asset when it is created and cannot be changed afterwards.

They are important because they allow assets to be uniquely identified and referenced, even if their names or locations change.

> They play an important role in networked multiplayer games, where they are used to ensure that all clients have the same asset data and can synchronize their gameplay correctly.

Unreal Engine represents GUIDs are strings in the format `"{01234567-89AB-CDEF-0123-456789ABCDEF}"`.


### File Signature

The first 4 bytes of the `.sav` file is: 

```
[0x47, 0x56, 0x41, 0x53]
```

which decodes into the string `"GVAS"`.

### File Metadata

