// Copyright 1998-2019 Epic Games, Inc. All Rights Reserved.
#include "ThirdPersonMPGameMode.h"
#include "ThirdPersonMPCharacter.h"
#include "UObject/ConstructorHelpers.h"

#include "Engine/Engine.h"


#include "adaptive.h"

AThirdPersonMPGameMode::AThirdPersonMPGameMode()
{
	// set default pawn class to our Blueprinted character
	static ConstructorHelpers::FClassFinder<APawn> PlayerPawnBPClass(TEXT("/Game/ThirdPersonCPP/Blueprints/ThirdPersonCharacter"));
	if (PlayerPawnBPClass.Class != NULL)
	{
		DefaultPawnClass = PlayerPawnBPClass.Class;
	}
}

void AThirdPersonMPGameMode::InitGame(const FString& MapName, const FString& Options, FString& ErrorMessage)
{
	AGameModeBase::InitGame(MapName, Options, ErrorMessage);

	if (!solver_.start_server())
	{
		FString healthMessage = TEXT("Server failed");
		GEngine->AddOnScreenDebugMessage(-1, 5.f, FColor::Red, healthMessage);
	}
	else
	{
		FString healthMessage = TEXT("Server started");
		GEngine->AddOnScreenDebugMessage(-1, 5.f, FColor::Green, healthMessage);
	}
}

AdaptiveSolver& AThirdPersonMPGameMode::get_solver_mut()
{
	return solver_;
}

