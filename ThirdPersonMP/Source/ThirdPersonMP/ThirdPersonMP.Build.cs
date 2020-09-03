// Copyright 1998-2019 Epic Games, Inc. All Rights Reserved.

using UnrealBuildTool;

using System;
using System.IO;

public class ThirdPersonMP : ModuleRules
{
	public ThirdPersonMP(ReadOnlyTargetRules Target) : base(Target)
	{
		PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;

		PublicDependencyModuleNames.AddRange(new string[] { "Core", "CoreUObject", "Engine", "InputCore", "HeadMountedDisplay" });
        var root = Path.Combine(ModuleDirectory, "..", "..", "..", "game_server");
        root = Path.GetFullPath(root);

        var lib_location = Path.Combine(root, "target", "release");
        var dll_lib_location = Path.Combine(lib_location, "game_server.dll.lib");
        var dll_location = Path.Combine(lib_location, "game_server.dll");

        var header_location = Path.Combine(root, "export", "header");

        Console.WriteLine("Root: " + root);
        Console.WriteLine("lib: " + dll_lib_location);
        Console.WriteLine("dll: " + dll_location);
        Console.WriteLine("header: " + header_location);

        PublicAdditionalLibraries.Add(dll_lib_location);
	    PublicIncludePaths.Add(header_location);
        RuntimeDependencies.Add("$(TargetOutputDir)\\game_server.dll",  dll_location);
    }
}
