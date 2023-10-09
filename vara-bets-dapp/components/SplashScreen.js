import React, { useEffect, useRef } from 'react';
import { View, Animated, StyleSheet } from 'react-native';
import * as Animatable from 'react-native-animatable';

const SplashScreen = () => {
  const fadeAnim = useRef(new Animated.Value(0)).current;

  useEffect(() => {
    Animated.timing(fadeAnim, {
      toValue: 1,
      duration: 4000,
      useNativeDriver: true,
    }).start();
  }, []);

  return (
    <View style={styles.container}>
      <Animated.View style={[styles.fadeContainer, { opacity: fadeAnim }]}>
        <Animatable.Text
          animation="bounceIn"
          iterationCount="infinite"
          style={styles.text}
        >
          Welcome to Vara-Bets!
        </Animatable.Text>
      </Animated.View>
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
  },
  fadeContainer: {
    width: '100%',
    alignItems: 'center',
  },
  text: {
    fontSize: 24,
    fontWeight: 'bold',
  },
});

export default SplashScreen;