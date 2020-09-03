// Fill out your copyright notice in the Description page of Project Settings.


#include "AdaptiveSolver.h"

AdaptiveSolver::AdaptiveSolver()
	: server_(adaptive::server_new())
{
}

AdaptiveSolver::~AdaptiveSolver()
{
	adaptive::server_free(server_);
}

bool AdaptiveSolver::has_data() const
{
	return adaptive::server_has_data(server_);
	//return false;
}

bool AdaptiveSolver::start_server(uint16_t port)
{
	return adaptive::server_start(server_, port);
}

bool AdaptiveSolver::get_coordinate(double* x, double* y, double* z)
{
	return adaptive::server_get_coord(server_,  x, y, z);
}

bool AdaptiveSolver::send_data(double data)
{
	return adaptive::server_send_data(server_, data);
}
