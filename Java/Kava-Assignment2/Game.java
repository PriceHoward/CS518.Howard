
import java.util.HashMap;
import java.util.Map;

public class Game {

    Map<Integer, Frame> FrameScore = new HashMap<>();
    Integer frameNumber = 0;
    Integer rollNumber = 1;

    // Tries to locate previous frame object in the map. If not found makes a new one.
    Frame CreateandGetFrameObject() {
        Frame newFrame;
        newFrame = FrameScore.get(frameNumber);
        if (newFrame == null) {
            return newFrame = new Frame();
        }
        return newFrame;
    }

    //Function for the actual rolling of the ball and setting the frame object.
    public void roll(int pinsKnockedDown) {
        Frame newFrame = CreateandGetFrameObject();
        FrameScore.put(frameNumber, newFrame);

        if (frameNumber == 9) // If Frame 10
        {
            switch (rollNumber) {

                case 1 -> { // Roll One
                    if (pinsKnockedDown == 10) { // Strike
                        newFrame.bowl1 = pinsKnockedDown;
                        newFrame.bonus[0] = 'S';
                    } else {
                        newFrame.bowl1 = pinsKnockedDown;
                    }
                    rollNumber++;
                }

                case 2 -> { // Roll Two
                    if (pinsKnockedDown == 10) { // Strike
                        newFrame.bowl2 = pinsKnockedDown;
                        newFrame.bonus[1] = 'S';
                    } else {
                        newFrame.bowl2 = pinsKnockedDown;
                        if (newFrame.bowl1 + newFrame.bowl2 == 10) {
                            newFrame.bonus[1] = 'P';
                        }
                    }
                    rollNumber++;
                }

                default -> { // Roll Three
                    if (newFrame.bonus[1] == 'S' || newFrame.bonus[1] == 'P') {
                        newFrame.bowl3 = pinsKnockedDown;
                    }
                }
            }
        } // End of Frame 10 Logic
        else if (rollNumber == 1) {
            if (pinsKnockedDown == 10) { // Strike
                newFrame.bowl1 = pinsKnockedDown;
                newFrame.bowl2 = 0;
                newFrame.bonus[0] = 'S';
                frameNumber++;
            } else { // Normal Bowl.
                newFrame.bowl1 = pinsKnockedDown;
                rollNumber++;
            }
        } // End of Strike Logic
        else {
            if (newFrame.bowl1 + pinsKnockedDown == 10) { // Spare
                newFrame.bowl2 = pinsKnockedDown;
                newFrame.bonus[0] = 'P';
                rollNumber = 1;
            } else { // Normal Bowl.
                newFrame.bowl2 = pinsKnockedDown;
            }
            frameNumber++;
            rollNumber = 1;
        }
    }

    public int score() {
        int score = 0;

        for (int i = 0; i < FrameScore.size(); i++) {
            int calculateStrikeSpare = 0;

            if (i != 9) {
                switch (FrameScore.get(i).bonus[0]) {

                    case 'S' -> {
                        calculateStrikeSpare += FrameScore.get(i).bowl1;
                        if (FrameScore.get(i + 1).bonus[0] == 'S') {
                            calculateStrikeSpare += FrameScore.get(i + 1).bowl1;
                            if (i != 8) {
                                calculateStrikeSpare += FrameScore.get(i + 2).bowl1;
                            } else {
                                calculateStrikeSpare += FrameScore.get(i + 1).bowl2;
                            }
                        } else {
                            calculateStrikeSpare += FrameScore.get(i + 1).bowl1;
                            calculateStrikeSpare += FrameScore.get(i + 1).bowl2;
                        }
                        score += calculateStrikeSpare;
                    }

                    case 'P' -> {
                        calculateStrikeSpare += FrameScore.get(i).bowl1 + FrameScore.get(i).bowl2;
                        calculateStrikeSpare += FrameScore.get(i + 1).bowl1;
                        score += calculateStrikeSpare;
                    }

                    default -> {
                        score += FrameScore.get(i).bowl1 + FrameScore.get(i).bowl2;
                    }
                }
            } else {
                Frame frameTen = FrameScore.get(i);

                if (frameTen.bonus[0] == 'S') {
                    if (frameTen.bonus[1] == 'S' && frameTen.bonus[2] == 'S') {
                        calculateStrikeSpare += 30;
                    } else {
                        calculateStrikeSpare += frameTen.bowl1 + frameTen.bowl2 + frameTen.bowl3;
                    }
                } else {
                    calculateStrikeSpare += frameTen.bowl1;
                    if (frameTen.bonus[1] == 'S' || frameTen.bonus[1] == 'P') {
                        calculateStrikeSpare += frameTen.bowl2 + frameTen.bowl3;
                    } else {
                        calculateStrikeSpare += frameTen.bowl2;
                    }
                }
                score += calculateStrikeSpare;
            }
        }
        return score;
    }
} // End of Game Object

class Frame {

    int bowl1;
    int bowl2;
    int bowl3;
    char[] bonus; // S for strike P for spare.

    public Frame() {
        this.bowl1 = 0;
        this.bowl2 = 0;
        this.bowl3 = 0;
        this.bonus = new char[3];
        this.bonus[0] = ' ';
        this.bonus[1] = ' ';
        this.bonus[2] = ' ';
    }
}
