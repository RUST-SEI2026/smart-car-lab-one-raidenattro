#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pose {
    pub x: i32,
    pub y: i32,
    pub heading: char,
}

impl Pose {
    pub fn new(x: i32, y: i32, heading: char) -> Self {
        Pose { x, y, heading }
    }
}

impl Default for Pose {
    fn default() -> Self {
        Pose {
            x: 0,
            y: 0,
            heading: 'N',
        }
    }
}

pub struct Executor {
    pose: Pose,
}

impl Executor {
    pub fn with_pose(pose: Pose) -> Self {
        Executor { pose }
    }

    // pub fn execute(&mut self, cmds: &str) {
    //     for cmd in cmds.chars() {
    //         match cmd {
    //             'M' => match self.pose.heading {
    //                 'N' => self.pose.y += 1,
    //                 'S' => self.pose.y -= 1,
    //                 'E' => self.pose.x += 1,
    //                 'W' => self.pose.x -= 1,
    //                 _ => {} // 忽略非法朝向
    //             },
    //             'L' => match self.pose.heading {
    //                 'N' => self.pose.heading = 'W',
    //                 'W' => self.pose.heading = 'S',
    //                 'S' => self.pose.heading = 'E',
    //                 'E' => self.pose.heading = 'N',
    //                 _ => {}
    //             },
    //             'R' => match self.pose.heading {
    //                 'N' => self.pose.heading = 'E',
    //                 'E' => self.pose.heading = 'S',
    //                 'S' => self.pose.heading = 'W',
    //                 'W' => self.pose.heading = 'N',
    //                 _ => {}
    //             },
    //             _ => {} // 根据约束，假设参数均合法，此处忽略非MLR的其他字符
    //         }
    //     }
    // }

    pub fn query(&self) -> Pose {
        self.pose
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_default_pose() {
        let executor = Executor::with_pose(Pose::default());
        assert_eq!(executor.query(), Pose { x: 0, y: 0, heading: 'N' });
    }

    #[test]
    fn test_init_custom_pose() {
        let executor = Executor::with_pose(Pose::new(5, -3, 'E'));
        assert_eq!(executor.query(), Pose { x: 5, y: -3, heading: 'E' });
    }

    #[test]
    fn test_move_forward() {
        let mut executor = Executor::with_pose(Pose::default()); // (0,0,N)
        executor.execute("M");
        assert_eq!(executor.query(), Pose { x: 0, y: 1, heading: 'N' });

        executor.execute("R"); // 面向东 (E)
        executor.execute("M");
        assert_eq!(executor.query(), Pose { x: 1, y: 1, heading: 'E' });
    }

    #[test]
    fn test_turn_left() {
        let mut executor = Executor::with_pose(Pose::default()); // 初始面向N
        executor.execute("L");
        assert_eq!(executor.query(), Pose { x: 0, y: 0, heading: 'W' });
        executor.execute("L");
        assert_eq!(executor.query(), Pose { x: 0, y: 0, heading: 'S' });
        executor.execute("L");
        assert_eq!(executor.query(), Pose { x: 0, y: 0, heading: 'E' });
        executor.execute("L");
        assert_eq!(executor.query(), Pose { x: 0, y: 0, heading: 'N' });
    }

    #[test]
    fn test_turn_right() {
        let mut executor = Executor::with_pose(Pose::default()); // 初始面向N
        executor.execute("R");
        assert_eq!(executor.query(), Pose { x: 0, y: 0, heading: 'E' });
        executor.execute("R");
        assert_eq!(executor.query(), Pose { x: 0, y: 0, heading: 'S' });
        executor.execute("R");
        assert_eq!(executor.query(), Pose { x: 0, y: 0, heading: 'W' });
        executor.execute("R");
        assert_eq!(executor.query(), Pose { x: 0, y: 0, heading: 'N' });
    }

    #[test]
    fn test_batch_commands() {
        // 测试连续的批量指令
        let mut executor = Executor::with_pose(Pose::default());
        // 动作分解:
        // M -> (0, 1, 'N')
        // M -> (0, 2, 'N')
        // R -> (0, 2, 'E')
        // M -> (1, 2, 'E')
        // L -> (1, 2, 'N')
        // M -> (1, 3, 'N')
        executor.execute("MMRMLM");
        assert_eq!(executor.query(), Pose { x: 1, y: 3, heading: 'N' });
    }
}