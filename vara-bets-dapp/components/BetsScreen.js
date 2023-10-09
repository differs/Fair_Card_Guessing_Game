import React, { useState } from 'react';
import { Alert, View, Text, StyleSheet, TextInput, Button } from 'react-native';
// import { useState } from 'react';



  // return (
  // );

const BetsScreen = () => {
  const [text, setText] = useState('');

  return (
    <View style={styles.container}>
      <Text>vara bets</Text>


      <View>
        <Text>A fair OnChain card guess game build on vara chain.</Text>
      </View>


      <View style={{padding: 10}}>
        <TextInput
          style={{height: 40}}
          placeholder="Type What amount you will use."
          onChangeText={text => setText(text)}
          defaultValue={text}
        />
      </View>

      <View>
        <TextInput>1</TextInput>
        <TextInput>2</TextInput>
        <TextInput>3</TextInput>
        <TextInput>4</TextInput>
      </View>

      <View>
      <Button
        onPress={() => {
          Alert.alert('你点击了按钮！');
        }}
        title="点我！"
      />      
      </View>
  

    </View>


  );
};

export default BetsScreen;

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    justifyContent: 'center',
    // fontSize: 31,
  },
});