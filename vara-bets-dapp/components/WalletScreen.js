import React from 'react';
import { View, Text, StyleSheet } from 'react-native';

const WalletsScreen = () => {
  return (
    <View style={styles.container}>
      <Text>Wallets Screen</Text>
    </View>
  );
};

export default WalletsScreen;

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    justifyContent: 'center',
  },
});