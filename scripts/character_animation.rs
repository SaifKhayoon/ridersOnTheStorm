#include "AnimInstance.h"

#define ANIM_MONTAGE_SLOT_NAME "UpperBody"

class UCharacterAnimation : public UObject {
public:
  UCharacterAnimation() {
    CurrentAnim = NULL;
  }

  UAnimMontage* CurrentAnim;

  UAnimInstance* AnimInstance;

  void PlayAnim(UAnimMontage* Montage, float InPlayRate = 1.f, FName StartSectionName = NAME_None) {
    if (AnimInstance && Montage) {
      FName SlotName = ANIM_MONTAGE_SLOT_NAME;
      CurrentAnim = Montage;
      AnimInstance->Montage_Play(Montage, InPlayRate, EMontagePlayReturnType::MontageLength, 0.f, StartSectionName, false, SlotName);
    }
  }

  void StopAnim() {
    if (AnimInstance) {
      FName SlotName = ANIM_MONTAGE_SLOT_NAME;
      AnimInstance->Montage_Stop(CurrentAnim->BlendOut.GetBlendTime(), SlotName);
    }
  }

  bool IsAnimPlaying() {
    if (AnimInstance && CurrentAnim) {
      FName SlotName = ANIM_MONTAGE_SLOT_NAME;
      return AnimInstance->Montage_IsPlaying(CurrentAnim, SlotName);
    }
    return false;
  }
};

