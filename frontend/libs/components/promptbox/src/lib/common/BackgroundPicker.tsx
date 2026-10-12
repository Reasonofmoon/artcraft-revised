import { BlendIcon } from "lucide-react";
import { PopoverItem, PopoverMenu } from "@storyteller/ui-popover";
import { Tooltip } from "@storyteller/ui-tooltip";
import { CommonImageBackground } from "@storyteller/model-list";

interface BackgroundPickerProps {
  currentBackground?: CommonImageBackground;
  handleBackgroundSelect: (selected: CommonImageBackground) => void;
}

const BACKGROUND_OPTIONS: CommonImageBackground[] = [
  CommonImageBackground.Transparent,
  CommonImageBackground.Auto,
  CommonImageBackground.Opaque,
];

const BACKGROUND_LABELS: Record<CommonImageBackground, string> = {
  [CommonImageBackground.Auto]: "Auto",
  [CommonImageBackground.Transparent]: "Transparent",
  [CommonImageBackground.Opaque]: "Opaque",
};

/**
 * GPT Image background. Transparent is the default so generated images have
 * no background. Auto and Opaque stay available.
 */
export const BackgroundPicker = ({
  currentBackground,
  handleBackgroundSelect,
}: BackgroundPickerProps) => {
  const selected = currentBackground ?? CommonImageBackground.Transparent;

  const handleSelectAdapter = (item: PopoverItem) => {
    const background = (item.action ?? item.label) as CommonImageBackground;
    if (background) handleBackgroundSelect(background);
  };

  const items: PopoverItem[] = BACKGROUND_OPTIONS.map((background) => ({
    label: BACKGROUND_LABELS[background],
    action: background,
    selected: selected === background,
  }));

  return (
    <Tooltip
      content="Background"
      position="top"
      className="z-50"
      closeOnClick={true}
    >
      <PopoverMenu
        items={items}
        onSelect={handleSelectAdapter}
        mode="toggle"
        panelTitle="Background"
        triggerIcon={<BlendIcon className="h-3.5 w-3.5" />}
      />
    </Tooltip>
  );
};
