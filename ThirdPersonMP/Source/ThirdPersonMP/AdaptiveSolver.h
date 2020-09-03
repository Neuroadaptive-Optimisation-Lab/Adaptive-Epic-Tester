// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"

#include "adaptive.h"

/**
 * 
 */
class THIRDPERSONMP_API AdaptiveSolver
{
public:
	AdaptiveSolver();
	~AdaptiveSolver();

	bool has_data() const;
	bool start_server(uint16_t port=adaptive::DEFAULT_PORT);
	bool get_coordinate(double* x, double* y, double* z);
	bool send_data(double data);
private:
	adaptive::GameServer* server_;
};
