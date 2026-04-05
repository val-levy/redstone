.PHONY: help dev

help:
	@echo "Available commands:"
	@echo "  make dev        # Start development environment"

dev:
	docker-compose up --build
