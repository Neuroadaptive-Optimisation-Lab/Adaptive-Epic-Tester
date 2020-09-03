// Copyright 1998-2019 Epic Games, Inc. All Rights Reserved.

#pragma once

#include "AdaptiveSolver.h"

#include "CoreMinimal.h"
#include "GameFramework/GameModeBase.h"
#include "ThirdPersonMPCharacter.h"
#include "ThirdPersonMPGameMode.generated.h"


UCLASS(minimalapi)
class AThirdPersonMPGameMode : public AGameModeBase
{
	GENERATED_BODY()

public:
	AThirdPersonMPGameMode();

	void InitGame(const FString& MapName, const FString& Options, FString& ErrorMessage) override;

	AdaptiveSolver& get_solver_mut();
private:
	AdaptiveSolver solver_;
};



