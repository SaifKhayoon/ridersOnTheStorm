#include "GameFramework/CharacterMovementComponent.h"

#define CHARACTER_MOVEMENT_SPEED 500.0f

class UCharacterMovement : public UObject {
public:
  UCharacterMovement(class ACharacter* InCharacter) {
    Character = InCharacter;
    MovementComponent = Cast<UCharacterMovementComponent>(Character->GetMovementComponent());
  }

  ACharacter* Character;
  UCharacterMovementComponent* MovementComponent;

  void MoveForward(float Value) {
    if (MovementComponent && (MovementComponent->IsMovingOnGround() || MovementComponent->IsFalling())) {
      const FVector Direction = FRotationMatrix(Character->GetControlRotation()).GetScaledAxis(EAxis::X);
      Character->AddMovementInput(Direction, Value);
    }
  }

  void MoveRight(float Value) {
    if (MovementComponent && (MovementComponent->IsMovingOnGround() || MovementComponent->IsFalling())) {
      const FVector Direction = FRotationMatrix(Character->GetControlRotation()).GetScaledAxis(EAxis::Y);
      Character->AddMovementInput(Direction, Value);
    }
  }

  void Jump() {
    if (MovementComponent && MovementComponent->IsMovingOnGround()) {
      MovementComponent->Jump();
    }
  }

  void StopJumping() {
    if (MovementComponent && MovementComponent->IsFalling()) {
      MovementComponent->StopJumping();
    }
  }
};

